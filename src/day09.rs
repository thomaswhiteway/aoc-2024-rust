use failure::Error;
use std::cmp::min;
use std::ops::Range;

trait Defrag {
    fn defragment(segments: &mut Vec<Segment>);
}

struct BlockDefrag {}

impl Defrag for BlockDefrag {
    fn defragment(segments: &mut Vec<Segment>) {
        let mut index = 0;
        while index < segments.len() - 1 {
            let space = segments[index + 1].range.start - segments[index].range.end;
            if space != 0 {
                let next_free = segments[index].range.end;
                let new_segment = segments.last_mut().unwrap().split_to(next_free, space);
                segments.insert(index + 1, new_segment);
                if segments.last().unwrap().range.is_empty() {
                    segments.pop();
                }
            }

            index += 1;
        }
    }
}

struct FileDefrag {}

impl Defrag for FileDefrag {
    fn defragment(segments: &mut Vec<Segment>) {
        for file_id in (0..segments.len() as u64).rev() {
            let segment_index = segments
                .iter()
                .enumerate()
                .find_map(|(index, segment)| {
                    if segment.file_id == file_id {
                        Some(index)
                    } else {
                        None
                    }
                })
                .unwrap();
            let segment = segments[segment_index].clone();

            for index in 0..segment_index {
                let space = segments[index + 1].range.start - segments[index].range.end;

                if space >= segment.size() {
                    let next_free = segments[index].range.end;
                    let new_segment = segment.move_to(next_free);
                    segments.remove(segment_index);
                    segments.insert(index + 1, new_segment);

                    break;
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Segment {
    file_id: u64,
    range: Range<u64>,
}

impl Segment {
    fn checksum(&self) -> u64 {
        self.range.clone().map(|index| index * self.file_id).sum()
    }

    fn size(&self) -> u64 {
        self.range.end - self.range.start
    }

    fn split_to(&mut self, position: u64, max_amount: u64) -> Self {
        let amount = min(max_amount, self.size());
        self.range.end -= amount;
        Segment {
            file_id: self.file_id,
            range: position..position + amount,
        }
    }

    fn move_to(&self, position: u64) -> Self {
        Segment {
            file_id: self.file_id,
            range: position..position + self.size(),
        }
    }
}

fn calculate_checksum(segments: &Vec<Segment>) -> u64 {
    segments.iter().map(|segment| segment.checksum()).sum()
}

fn defragmented_checksum<D: Defrag>(mut segments: Vec<Segment>) -> u64 {
    D::defragment(&mut segments);
    calculate_checksum(&segments)
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<Segment>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(data
            .trim()
            .char_indices()
            .scan(0u64, |pos, (index, digit)| {
                let width = digit.to_digit(10).unwrap() as u64;
                let start = *pos;
                *pos += width;

                if index % 2 == 0 {
                    Some(Some(Segment {
                        file_id: index as u64 / 2,
                        range: start..start + width,
                    }))
                } else {
                    Some(None)
                }
            })
            .flatten()
            .collect())
    }

    fn solve(segments: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = defragmented_checksum::<BlockDefrag>(segments.clone());
        let part2 = defragmented_checksum::<FileDefrag>(segments);
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
