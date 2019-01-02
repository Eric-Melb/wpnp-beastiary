// I did not fully understand what Rust being an "expressive" language meant when I wrote these
// they are now, obviously, unnecessary. As my first use of generics and traits in rust however
// they can stay

pub fn higher_of<T: PartialOrd>(first: T, second: T) -> T
{
        if first > second
        {
                return first;
        }
        else
        {
                return second;
        }
}

#[cfg(test)]
mod test_higher_of
{
        use helpers::higher_of;

        #[test]
        fn returns_higher_integer()
        {
                let high_number = 5;
                let low_number = 2;
                let result = higher_of(low_number,high_number);

                assert_eq!(high_number, result);
        }

        #[test]
        fn returns_higher_float()
        {
                let high_number = 5.0;
                let low_number = 2.0;
                let result = higher_of(low_number, high_number);

                assert_eq!(high_number, result);

        }
}

pub fn lower_of<T: PartialOrd>(first: T, second: T) -> T
{
        if first < second
        {
                return first;
        }
        else
        {
                return second;
        }

}

#[cfg(test)]
mod test_lower_of
{
        use helpers::lower_of;

        #[test]
        fn returns_lower_integer()
        {
                let high_number = 5;
                let low_number = 1;
                let result = lower_of(low_number, high_number);

                assert_eq!(low_number, result);
        }

        #[test]
        fn returns_lower_float()
        {
                let high_number = 5.0;
                let low_number = 2.0;
                let result = lower_of(low_number, high_number);

                assert_eq!(low_number, result);
        }
}


