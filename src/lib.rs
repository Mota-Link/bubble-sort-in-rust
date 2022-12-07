/// # 冒泡排序
///
/// 每轮比较相邻两位，把大的元素交换到高位。
/// 优化：当一次冒泡没有元素交换行为，那说明已经按顺序了，可以提前退出。
///
/// ```
/// # use bubble_sort::bubble_sort;
/// #
/// let mut vec_0: Vec<i32> = Vec::new();
/// let mut vec_1 = vec![1];
/// let mut vec_2 = vec![3,5,2,4,1];
/// let mut vec_3 = vec!['q','w','e','r'];
///
/// bubble_sort(&mut vec_0);
/// bubble_sort(&mut vec_1);
/// bubble_sort(&mut vec_2);
/// bubble_sort(&mut vec_3);
///
/// assert_eq!(vec_0, Vec::new());
/// assert_eq!(vec_1, vec![1]);
/// assert_eq!(vec_2, vec![1,2,3,4,5]);
/// assert_eq!(vec_3, vec!['e','q','r','w']);
/// ```
///
/// 
pub fn bubble_sort<T>(vec: &mut Vec<T>)
where
    T: PartialOrd,
{
    // 冒泡行为标志
    let mut is_bubbled;

    for i in 0..vec.len() {
        is_bubbled = false;

        // 模拟窗口迭代
        for (x, y) in (1..(vec.len() - i)).enumerate() {
            if vec[x] > vec[y] {
                // let temp = vec[x];
                // vec[x] = vec[y];
                // vec[y] = temp;
                vec.swap(x, y);

                is_bubbled = true;
            }
        }

        // 若本轮没有冒泡行为，说明已排好序，可提前退出
        if !is_bubbled {
            break;
        }
    }
}