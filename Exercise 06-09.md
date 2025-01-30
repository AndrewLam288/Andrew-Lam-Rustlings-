# Do Rustlings Sections 00 (Intro) to 05 (Vecs)

## Exercise 0.7 structs2

## Code Snippeet

#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();

        // TODO: Create your own order using the update syntax and template above!
        // let your_order =

        let your_order = Order {
            name: String::from("Hacker in Rust"),
            count: 1,
            ..order_template
        };
        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}

## Code Explanation

The code include a struct definition for Order with various fields and a function to create an Order template, which can be use to create new Order. The code begin with calling let your_order = Order { }; which is the creation of new Order instance using the struct with the name field, setting the count field and copies the remaining field from order_template that have been given into the new Order instance.

## My own write up

This was pretty simple but somehow I over complicated myself, I try to put count: 1 after ..order_template but it seems not to work as it wouldn't allow a comma after the base struct. The base struct need to be the last field that got put in.

I simply changed from:
let your_order = Order {
            name: String::from("Hacker in Rust"),
            // count: 1,
            ..order_template,
            count: 1,
        };
to: let your_order = Order {
            name: String::from("Hacker in Rust"),
            count: 1,
            ..order_template
        };

I just want to note this as it took me a while, mostly cause I didn't read carefully about the warning that Rust gave me. But are there any other scenario where it would fall off this base struct, like the base struct not being the final field. Also I was wondering what is the #[cfg(test)] and the following mod tests, use super::*; was for
