fn main() {
    let filter_condition = FilterCondition{num: 87};
    let collection = vec![56,78,98,87,24,65,87];

    println!("{:?}",custom_filter(collection, filter_condition));
}

struct FilterCondition{
    num: i32,
}

impl FilterCondition{
    fn is_match(&self, n: &i32) -> bool {
        if *n >= self.num{
            true
        }
        else{
            false
        }
    }
}

fn custom_filter(collection: Vec<i32>, filter: FilterCondition) -> Vec<i32>{
    let mut new_collection: Vec<i32> = Vec::new();
    for number in collection{
        if filter.is_match(&number){
            new_collection.push(number)
        }
    }
    new_collection
}