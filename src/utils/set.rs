// pub struct Set<T> 
// where T: std::cmp::PartialEq + Copy {
//     elements: Vec<T>,
// }

// impl<T> Set<T>
// where T: std::cmp::PartialEq + Copy {
//     pub fn new(elements: Vec<T>) -> Self {
//         Set { elements }
//     }

//     pub fn elements(&self) -> &Vec<T> {
//         &self.elements
//     }

//     pub fn sample(&self, index: Option<usize>) -> Option<&T> {
//         let idx = index.unwrap_or(0);
//         self.elements().get(idx)
//     }

//     pub fn contains(&self, element: &T) -> bool {
//         self.elements().contains(element)
//     }
// }

pub trait Set<T> {
    fn new_set(elements: Vec<T>) -> Self;
    fn elements(&self) -> &Vec<T>;
    fn sample(&self, index: Option<usize>) -> Option<&T>;
    fn contains(&self, element: &T) -> bool;
}