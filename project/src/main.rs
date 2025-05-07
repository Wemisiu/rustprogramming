
#[cfg(test)]
mod tests {
   
    #[test]

    fn testing_function() {



    }

    
     
    
}


fn concept_of_shadowing_mixed_up(){
    // sometimes you will see next usage pattern
    fn get_score() -> i32{
        return 100;
    }

    let result = get_score();
    print!("Student scored: {}. ",result);
    let result = result > 90;
    println!("IsPassed: {}",result);
    

}




fn main() {


    concept_of_shadowing_mixed_up();
    
    


}
