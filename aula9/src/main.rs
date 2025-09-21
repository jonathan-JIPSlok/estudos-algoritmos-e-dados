use bubble_sort::rodar as rodar_bubble;
use quick_sort::rodar as rodar_quick;
use merge_sort::rodar as rodar_merge;

fn main() {

    // Bubble Sort
    println!("{:-<75}", "");
    println!("{:^75}", "Bubble Sort");
    println!("{:-<75}", "");

    rodar_bubble();

    // Quick Sort
    println!("{:-<75}", "");
    println!("{:^75}", "Quick Sort");
    println!("{:-<75}", "");

    rodar_quick();

    // Merge Sort
    println!("{:-<75}", "");
    println!("{:^75}", "Merge Sort");
    println!("{:-<75}", "");

    rodar_merge();
}