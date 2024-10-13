mod variables; // Імпорт модуля variables

mod basic_types { // Оголошення модуля basic_types
    pub mod numbers; // Оголошення модуля numbers
    pub mod char_bool;
    pub mod statements_expressions;
    pub mod functions;
}
use crate::basic_types::numbers;
use crate::basic_types::char_bool;
use crate::basic_types::statements_expressions;
use crate::basic_types::functions;



mod ownership_borrowing {
    pub mod ownership;
    pub mod reference_borrowing;
}

use crate::ownership_borrowing::ownership;
use crate::ownership_borrowing::reference_borrowing;

mod compound_types{
    pub mod string;
    pub mod array;
    pub mod slice;
    pub mod tuple;
    pub mod r#struct;

}

use crate::compound_types::string;
use crate::compound_types::array;
use crate::compound_types::slice;
use crate::compound_types::tuple;
use crate::compound_types::r#struct;

fn main() {
    // Виклик функцій з модуля variables
    variables::task1();
    variables::task2();
    variables::task3();
    variables::task4();
    variables::task5();
    variables::task6();
    variables::task7();
    variables::task8();
    variables::task9();


    numbers::task1();
    numbers::task2();
    numbers::task3();
    numbers::task4();
    numbers::task5();
    numbers::task6();
    numbers::task7();
     numbers::task8();
     numbers::task9();
    numbers::task10();
    numbers::task11();


    char_bool::task1();
    char_bool::task2();
    char_bool::task3();
    char_bool::task4();
    char_bool::task5();
    char_bool::task6();

    statements_expressions::task1();
    statements_expressions::task2();
    statements_expressions::task3();

    functions::task1();
    functions::task2();
    functions::task3();
    functions::task4();
    functions::task5();

    ownership::task1();
    ownership::task2();
    ownership::task3();
    ownership::task4();
    ownership::task5();
    ownership::task6();
    ownership::task7();
    ownership::task8();
    ownership::task9();


    reference_borrowing::task1();
    reference_borrowing::task2();
    reference_borrowing::task3();
    reference_borrowing::task4();
    reference_borrowing::task5();
    reference_borrowing::task6();
    reference_borrowing::task7();
    reference_borrowing::task8();
    reference_borrowing::task9();
    reference_borrowing::task10();
    reference_borrowing::task11();

    string::task1();
    string::task2();
    string::task3();
    string::task4();
    string::task5();
    string::task6();
    string::task7();
    string::task8();
    string::task9();
    string::task10();
    string::task11();
    string::task12();


    array::task1();
    array::task2();
    array::task3();
    array::task4();
    array::task5();
    array::task6();

    slice::task1();
    slice::task2();
    slice::task3();
    slice::task4();
    slice::task5();
    slice::task6();

    tuple::task1();
    tuple::task2();
    tuple::task3();
    tuple::task4();
    tuple::task5();
    tuple::task6();



    r#struct::task1();
    r#struct::task2();
    r#struct::task3();
    r#struct::task4();
    r#struct::task5();
    r#struct::task6();
    r#struct::task7();
    r#struct::task8();
}




