mod hard;
mod junior_algorithm;
mod medium;
mod simple;

fn main() {
  use medium::c725::ListNode;
  println!(
    "{:?}",
    medium::c725::test(
      Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
          val: 2,
          next: Some(Box::new(ListNode { val: 3, next: None }))
        }))
      })),
      5
    )
  )
}
