use egg::{define_language, Id, Symbol};

pub type RecExpr = egg::RecExpr<POS>;

define_language! {
    pub enum POS {
       "sentence" = List(Box<[Id]>),
       "sub" = Subject(Id),
       "d_obj" = DirectObj(Id),
       "i_obj" = IndirectObj(Id),
       "verb" = Verb(Id),
       "adj" = Adjective(Id),
       "adv" = Adverb(Id),
       "emp" = Emphasis(Id),
       Word(Symbol),
    }
}
