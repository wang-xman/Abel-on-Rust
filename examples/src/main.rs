//! Examples for Abel-on-Rust library
//! 
//! Focus here is how to use JSON and JSON plus parsers and loaders.
//! Codes in this crate are raw and commnents are verbose, which walks
//! the user through the "parsing-loading-getting" process.
//! 
//! All JSON files (including JSON+ format) are stored in the same
//! directory as `main.rs`. The "current directory" is the parent
//! directory of folder `/src`
//! 
//! standard_json.json - A standard JSON file created according to JSON
//!     grammar.
//! 
//! json_plus.txt - A JSON+ format file. The extension `.txt` reminds
//!     us that it is not conforming to JSON grammar.
extern crate abel;

use abel::object::Object;
use abel::bool::Bool;
use abel::integer::Integer;
use abel::double::Double;
use abel::complex::Complex;
use abel::text::Text;
use abel::container::Container;
use abel::list::List;
use abel::dict::Dict;
use abel::json_loader::JsonLoader;

// Example - Standard JSON
//
// Standard JSON accepts two types of containers, dictionary (object) and
// list (array). Only four data types null, double, string (text), and bool.
//
// Input file : standard_json.json
fn standard_json_loader() {

    // Since we are now interested in loading standard JSON file,
    // let's create a dedicated JSON loader
    let mut json_loader = JsonLoader::new();
    json_loader.load_from_file("./src/standard_json.json");

    // We can check if the root container or global dictionary is
    // created as expected
    assert_eq!(json_loader.get_global_dict().type_name(), "Dict");

    // Inside this global dictionary, there is a root key "ROOT_KEY_"
    // and it points to a list container that stores the dictionary
    // in the input file. We can check
    let global = json_loader.get_global_dict();
    let root_list = global.get_ref::<List>("ROOT_KEY_").unwrap();
    // ... and the root container is indeed a List
    assert!(
        global.get_ref::<List>("ROOT_KEY_").unwrap().as_any().is::<List>()
    );

    // Root container, i.e the list has onlyt one element
    assert_eq!(root_list.len(), 1);
    // ... and that element is the desired target dictionary
    let target_dict = root_list.get_ref::<Dict>(0).unwrap();

    // Now, let's go through the loaded target dictionary. Please bear
    // with me the verbose syntax of Rust. We have to face the reality
    // that Rust is a strongly typed langugage. Don't be bothered by
    // the turbo fish as you may not need them in practice.
    // Note that method `get_ref` returns an immutable reference.
    //
    // Key "STR" points to a Text type, let's check
    assert!(target_dict.get_ref::<Text>("STR").unwrap().as_any().is::<Text>());
    // ... we may access the value by an immutable reference
    let entry_text = target_dict.get_ref::<Text>("STR").unwrap();
    assert!(entry_text == "REGISTER");
    // Key "DBL" is a double type of value 5
    let entry_double = target_dict.get_ref::<Double>("DBL").unwrap();
    // ... and note that type of `entry_double` is `&Double`
    assert!(entry_double == &5.0);
    // Key "BOOL" has a value of boolean type
    let entry_bool = target_dict.get_ref::<Bool>("BOOL").unwrap();
    assert!( entry_bool == &true);
    // Key "LIST" has a value that is a list,
    let entry_list = target_dict.get_ref::<List>("LIST").unwrap();
    // ... and it has 3 elements
    assert!(entry_list.len() == 3);
    // ... the first element is a double
    let double_number = entry_list.get_ref::<Double>(0).unwrap();
    // ... and its value is 100
    assert!(double_number == &100.0);
    
}


// Example - JSON plus format
// 
// JSON plus format is more flexible. First of all, it allows comment lines
// inside the file. Outside a non-quoted string, anything after the character
// `#` is neglected.
//
// JSON plus format accepts more types. In additional to the types accepted
// by standard JSON, it also accepts integer, complex, binary, and bitstring.
//
// Input file : json_plus.txt
fn json_plus() {

    // This time we are now interested in loading standard JSON file,
    // let's create a dedicated JSON+ loader. Even though the type of
    // this loader is still `JsonLoader`, its field `parser_type` is
    // set to `json_plus`.
    let mut loader = JsonLoader::new_plus();
    loader.load_from_file("./src/json_plus.txt");

    // Inside the global dictionary, there is a root key "ROOT_KEY_"
    // and it points to a list container that stores the dictionary
    // in the input file. We can check
    let global = loader.get_global_dict();
    let root_list = global.get_ref::<List>("ROOT_KEY_").unwrap();
    // ... and the root container is indeed a List
    assert!(
        global.get_ref::<List>("ROOT_KEY_").unwrap().as_any().is::<List>()
    );
    // Root container, i.e the list has onlyt one element
    assert_eq!(root_list.len(), 1);
    // ... and that element is the desired target dictionary
    let target_dict = root_list.get_ref::<Dict>(0).unwrap();

    // Again, let's walk through the target dictionary we have just
    // acquired. I like to show those extended types that are not
    // available in JSON.

    // Let's start from the integer, a new addition
    let entry_int = target_dict.get_ref::<Integer>("INT1").unwrap();
    assert!(entry_int == &5);

    // The complex type is another new comer
    let entry_cmp1 = target_dict.get_ref::<Complex>("CPL1").unwrap();
    let cmp1: Complex = Complex::new(0.1,2.0);
    assert!(entry_cmp1 == &cmp1);
    // ... and there is another complex
    let entry_cmp2 = target_dict.get_ref::<Complex>("CPL2").unwrap();
    let cmp2: Complex = Complex::new(0.0,0.01);
    assert!(entry_cmp2 == &cmp2);


}

fn main() {
    println!(" EXAMPLE - Loading a standard JSON file ");
    standard_json_loader();
    println!(" EXAMPLE - Loading a file in JSON+ format ");
    json_plus();   
}
