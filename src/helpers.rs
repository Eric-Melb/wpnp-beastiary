pub fn higher_of<T: PartialOrd>(first: T, second: T) -> T
{
        if first > second
        {
                return first;
        } else {
                return second;
        }
}

pub fn lower_of<T: PartialOrd>(first: T, second: T) -> T
{
        if first < second
        {
                return first;
        } else {
                return second;
        }
}