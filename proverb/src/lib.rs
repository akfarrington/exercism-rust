pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb: String = "".to_string();
    if list.len() == 0 {
        return "".to_string();
    } else if list.len() ==1 {
        return "And all for the want of a ".to_string() + list[0] + &".".to_string();
    } else {
        let mut i = 0;
        while i < list.len(){
            if i < (list.len() - 1){
                proverb += &("For want of a ".to_string() + &list[i].to_string() + &" the ".to_string() + &list[i + 1].to_string() + &" was lost.\n".to_string());
            }
            i += 1;
        }
        proverb += &("And all for the want of a ".to_string() + list[0] + &".".to_string());
    }

    proverb
}
