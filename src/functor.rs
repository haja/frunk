pub trait Functor {
    type Contained;
    type Output<B> : Functor;
    fn map<F, B>(self, f: F) -> Self::Output<B>
    where F: Fn(Self::Contained) -> B;
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
    enum OptionFun<A> {
        Some(A),
        None,
    }

    impl <A> Functor for OptionFun<A> {
        type Contained = A;
        type Output<B> = OptionFun<B>;
        fn map<F, B>(self, f: F) -> Self::Output<B>
            where F: Fn(Self::Contained) -> B
        {
            match self {
                OptionFun::Some(x) => OptionFun::Some(f(x)),
                OptionFun::None => OptionFun::None
            }
        }
    }


    #[test]
    fn test_option_none() {
        let maybe = OptionFun::None;
        assert_eq!(maybe.map(|x: i32| x + 1), OptionFun::None)
    }

    #[test]
    fn test_option_num() {
        let maybe = OptionFun::Some(1);
        assert_eq!(maybe.map(|x| x + 1), OptionFun::Some(2));
    }

    #[test]
    fn test_option_str() {
        let maybe = OptionFun::Some("ab");
        assert_eq!(maybe.map(|x| x.len()), OptionFun::Some(2));
    }
}
