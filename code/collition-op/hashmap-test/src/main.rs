use std::collections::HashMap;

fn main() {
   let teams = vec![String::from("Blue"), String::from("Yellow")];
   let intial_scores = vec![10, 50];

   // 通过两个已经存在的集合创建HashMap
   let mut scores: HashMap<_,_> = teams.iter().zip(intial_scores.iter()).collect();
   let red_score = 70;
   let red = String::from("red");
   scores.insert(&red, &red_score);
   
   for (k, v) in &scores{
       println!("{}: {}", k , v);
   }

   // 如果不存在就插入
   let e = scores.entry(&red);
   let red_score = 90;
   e.or_insert(&red_score);

   for (k, v) in &scores{
       println!("{}: {}", k , v);
   }


   let field_name = String::from("color");
   let field_value = String::from("Blue");
   let mut map: HashMap<&str, &str> = HashMap::new();
   // 拥有所有权的值例如String，值会被移动，所有权会转移给HashMap
   // 将值的引用插入到HashMap，值本身就不会被移动
   // 在HashMap有效期间，被引用的值必须保持有效
   map.insert(&field_name, &field_value);
   println!("{}:{}", field_name, field_value);

   let text = "hello world wonderful world";
   let mut new_map: HashMap<&str, i32> = HashMap::new();
   // 将字符串按空格分割
   for word in text.split_whitespace(){
       // 如果当前单词首次出现就是0
       // 否则就会将值返回并加1
       let count = new_map.entry(word).or_insert(0);
       *count += 1;
   }
   println!("{:#?}", new_map)
}
