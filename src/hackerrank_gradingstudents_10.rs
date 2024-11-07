//https://www.hackerrank.com/challenges/grading/problem
fn gradingStudents(grades: &[i32]) -> Vec<i32>
{
    let mut rounded_grades = Vec::new();

    for &grade in grades
    {
        if grade < 38
        {
            rounded_grades.push(grade);
        }
        else
        {
            let next_multiple_of_5 = (grade / 5 + 1) * 5;
            if next_multiple_of_5 - grade < 3
            {
                rounded_grades.push(next_multiple_of_5);
            }
            else
            {
                rounded_grades.push(grade);
            }
        }
    }
    rounded_grades
}