// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(), //抄的，不太懂
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| capitalize_first(word)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
/*
输入一个字符串切片的向量，例如["hello", "world"]。
使用iter()得到每个元素的引用，也就是每个&str。
使用map对每个元素调用capitalize_first，将其转换为String。例如，"hello"变成"Hello"，"world"变成"World"。
收集所有的String到一个向量中，得到Vec<String>，如["Hello", "World"]。
使用join("")将这些字符串连接成一个大的字符串，中间没有分隔符，结果就是"HelloWorld"。
##################
capitalize_first 的作用:
将输入字符串的首字母大写，其余字符不变（如 "hello" → "Hello"）。
若输入为空字符串，返回空字符串。
类型转换:
map(|word| capitalize_first(word)) 将每个 &str 转换为 String。
join("") 将 Vec<String> 合并为单个 String。
*/
pub fn capitalize_words_string(words: &[&str]) -> String {
    words
        .iter()
        .map(|word| capitalize_first(word))
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
