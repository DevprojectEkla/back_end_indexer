///Cette structure permet de tokenizer le contenu d'un fichier au format vecteur de characteres,
///elle possede toutes les methodes pour examiner chaque char et en fait des tokens (des unités).
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    content: &'a [char], //lifetime 'a is needed for the whole struct
                         //when it has a field with a ref like this. the struct cannot outlive its reference
                         //the special lifetime 'static means a whole program lifetime
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }
    fn trim_left(&mut self) -> &'a [char] {
        //trim the blank space at the left of a token
        while self.content.len() > 0
            && (self.content[0].is_whitespace() || self.content[0].is_ascii_punctuation())
        {
            self.content = &self.content[1..];
        }
        self.content
    }
    fn trim_ascii_space(&mut self) -> &'a [char] {
        let mut n = 0;
        while n < self.content.len() && self.content[0].is_ascii_punctuation() {
            self.content = &self.content[1..];
            n += 1;
            // todo!("trim_ascii_space not IMPLEMENTED")
        }
        return self.content;
    }

    fn tokenize(&mut self, n: usize) -> &'a [char] {
        //after computing the n indice we call this function to get a token from the slice
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }

    fn token_while_condition<P>(&mut self, mut predicate: P) -> &'a [char]
    where
        P: FnMut(&char) -> bool,
        //this is non-trivial factorization of the code, basically it avoid repeating two while loop
        //but it is a bit obscure
        //keep in mind that it returns a self.tokenize(n) so it is just the same as the other if
        //condition but we add a while loop to it
        //with different condition
        //the beginning of the while loop on 'n' is always the same but the second member after '&&' changes. So we can pass this function to accept different predicate (=conditions). This predicate is a function which takes a &self.content[n] as an argument here, but where we call self.tokenizer_condition the argument is the condition on that previous &self.content argument i a closure like this: |x| x.is_alphanumeric()
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1; //
        }
        self.tokenize(n)
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        self.trim_ascii_space();

        if self.content.len() == 0 {
            return None;
        }
        if self.content[0].is_alphabetic() {
            // the idea is simple
            // content = ['h','e','l','l','o','2','2','1','\n']
            // we iterate on this because 'h' is alphabetic and we iterate over alphanumeric
            // character (A-Z, a-z, 0-9) and we stop at /n which is not
            // alpanumeric.
            //stops at rank n = 7 which is '1' in the example
            //and return ['h','e','l','l','o','2','2','1']
            //(the \n is gone)
            return Some(self.token_while_condition(|x| x.is_alphanumeric()));
        }
        if self.content[0].is_alphanumeric() {
            //tokenize numbers 442122 or also A12323
            return Some(self.token_while_condition(|x| !x.is_ascii_whitespace()));
        }
        return Some(self.tokenize(1));
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
