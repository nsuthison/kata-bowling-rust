struct Frame {
    first_roll: i32,
    second_roll: i32,
}

impl Frame {
    pub fn get_type(&self) -> FrameType {
        if self.first_roll == 10 {
            return FrameType::Strike;
        }

        if self.first_roll + self.second_roll == 10 {
            return FrameType::Spare;
        }

        FrameType::Score
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum FrameType {
    Score,
    Spare,
    Strike,
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn frame_get_type_should_return_strike_if_first_roll_score_equal_10() {
        let frame = Frame {
            first_roll: 10,
            second_roll: 0
        };

        assert_eq!(FrameType::Strike, frame.get_type());
    }

    #[test]
    fn frame_get_type_should_return_spare_if_first_roll_is_not_equal_to_10_and_summation_of_first_and_second_roll_equal_to_10() {
        let frame = Frame {
            first_roll: 4,
            second_roll: 6
        };

        assert_eq!(FrameType::Spare, frame.get_type());
    }

    #[test]
    fn frame_get_type_should_return_score_if_first_roll_is_not_equal_to_10_and_summation_of_first_and_second_roll_equal_to_10() {
        let frame = Frame {
            first_roll: 4,
            second_roll: 4
        };

        assert_eq!(FrameType::Score, frame.get_type());
    }
}
