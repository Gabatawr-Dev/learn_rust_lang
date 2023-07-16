#![forbid(unsafe_code)]

// рекурсивно для каждого первого элемента постепенно уменьшая размер массива
pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    if k == 0 {
        vec![vec![]]
    } else {
        let mut all_combinations = vec![];

        if arr.len() >= k {
            // рекурсивно получаем комбинации меньшего размера без первого элемента
            let without_first = combinations(&arr[1..], k - 1);
            
            // вклиниваем первый элемент в каждую комбинацию
            let with_first = without_first.into_iter()
                .map(|mut vec| {
                    vec.insert(0, arr[0]);
                    vec
                })
                .collect::<Vec<_>>();
            all_combinations.extend(with_first);

            // далее рекурсия остальной части массива по такому же приципу
            let without_first = combinations(&arr[1..], k);
            all_combinations.extend(without_first);
        }
        
        all_combinations
    }
}