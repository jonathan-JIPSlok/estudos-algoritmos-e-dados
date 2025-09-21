fn bubble_sort(arr: &mut [i32]) { // A função receve um array mutável
    let n = arr.len(); // Pegamos o tamanho do array
    for i in 0..n {
        // Após cada iteração, o maior elemento "sobe" para o final
        for j in 0..(n - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1); // Faz a conta entre um e outro e troca conforme o maior/menor
            }
        }
    }
}

pub fn rodar() {
    let mut array = [64, 34, 25, 12 ,22, 11, 90];
    println!("Array original: {:?}", array);

    bubble_sort(&mut array);
    println!("Bubble sort: {:?}", array);
}