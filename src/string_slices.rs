fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
   ///fdasfdsfdsffdsa
//// flkmdsalkfmdslkfmsdaklfmasldfmdsalkmflsdakmfldskmaflkmsdaflkmsdalfkmsadlfkmsadlkfmsdlakmflkmfdsafsdfsdafsdaf
/// fdsaf;;d;d[d[]]
/////fdsalf;dsa;lfadskg;dsalkadgs;lkg;ldsa;lkgs;dlkgfds;lakf;dslafkads;lfk;sdkfds;klf

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);///g,dfsgfaf;ldsa;lfads;ldfs;lfds;,la,;l

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
}