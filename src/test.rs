use std::collections::HashMap;
use std::hash::Hash;

pub fn main() {
    use glob::glob;
    use glob::glob_with;
    use glob::MatchOptions;

    let from = "/Users/yangjiaqi/Desktop/project/rust/my-learn-rust/src/**/*.*";

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    for entry in glob_with(from, options).unwrap() {
        if let Ok(path) = entry {
            println!("{:?}", path.display())
        }
    }

    fn group_by<T, V: Eq + Hash>(vec: Vec<T>, fun: &dyn Fn(&T) -> V) -> HashMap<V, Vec<T>> {
        let mut groups = HashMap::new();
        for v in vec.into_iter() {
            groups.entry(fun(&v)).or_insert(Vec::new()).push(v)
        }

        groups
    }

    let a = group_by(vec![11, 12, 2, 1, 2, 1, 2, 1231], &|v| v.to_string());
    println!("{:?}", a);

    fn group_by2<T, V, P>(vec: &Vec<T>, mut fun: P) -> HashMap<V, Vec<&T>>
    where
        V: Eq + Hash,
        P: FnMut(&T) -> V,
    {
        let mut groups = HashMap::new();
        for v in vec.iter() {
            groups.entry(fun(&v)).or_insert(Vec::new()).push(v)
        }
        groups
    }
}
