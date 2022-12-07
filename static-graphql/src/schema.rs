use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub struct Query;

macro_rules! define_object {
    ($name:ident) => {
        pub struct $name {
            value: i32,
        }

        impl $name {
            pub fn new(value: i32) -> Self {
                Self { value }
            }
        }

        #[async_graphql::Object]
        impl $name {
            async fn value(&self) -> i32 {
                self.value
            }
        }
    };
}
define_object!(Obj00);
define_object!(Obj01);
define_object!(Obj02);
define_object!(Obj03);
define_object!(Obj04);
define_object!(Obj05);
define_object!(Obj06);
define_object!(Obj07);
define_object!(Obj08);
define_object!(Obj09);
define_object!(Obj10);
define_object!(Obj11);
define_object!(Obj12);
define_object!(Obj13);
define_object!(Obj14);
define_object!(Obj15);
define_object!(Obj16);
define_object!(Obj17);
define_object!(Obj18);
define_object!(Obj19);
define_object!(Obj20);
define_object!(Obj21);
define_object!(Obj22);
define_object!(Obj23);
define_object!(Obj24);
define_object!(Obj25);
define_object!(Obj26);
define_object!(Obj27);
define_object!(Obj28);
define_object!(Obj29);
define_object!(Obj30);
define_object!(Obj31);
define_object!(Obj32);
define_object!(Obj33);
define_object!(Obj34);
define_object!(Obj35);
define_object!(Obj36);
define_object!(Obj37);
define_object!(Obj38);
define_object!(Obj39);
define_object!(Obj40);
define_object!(Obj41);
define_object!(Obj42);
define_object!(Obj43);
define_object!(Obj44);
define_object!(Obj45);
define_object!(Obj46);
define_object!(Obj47);
define_object!(Obj48);
define_object!(Obj49);
define_object!(Obj50);
define_object!(Obj51);
define_object!(Obj52);
define_object!(Obj53);
define_object!(Obj54);
define_object!(Obj55);
define_object!(Obj56);
define_object!(Obj57);
define_object!(Obj58);
define_object!(Obj59);
define_object!(Obj60);
define_object!(Obj61);
define_object!(Obj62);
define_object!(Obj63);
define_object!(Obj64);
define_object!(Obj65);
define_object!(Obj66);
define_object!(Obj67);
define_object!(Obj68);
define_object!(Obj69);
define_object!(Obj70);
define_object!(Obj71);
define_object!(Obj72);
define_object!(Obj73);
define_object!(Obj74);
define_object!(Obj75);
define_object!(Obj76);
define_object!(Obj77);
define_object!(Obj78);
define_object!(Obj79);
define_object!(Obj80);
define_object!(Obj81);
define_object!(Obj82);
define_object!(Obj83);
define_object!(Obj84);
define_object!(Obj85);
define_object!(Obj86);
define_object!(Obj87);
define_object!(Obj88);
define_object!(Obj89);
define_object!(Obj90);
define_object!(Obj91);
define_object!(Obj92);
define_object!(Obj93);
define_object!(Obj94);
define_object!(Obj95);
define_object!(Obj96);
define_object!(Obj97);
define_object!(Obj98);
define_object!(Obj99);

#[async_graphql::Object]
impl Query {
    /// Returns the sum of a and b
    async fn add00(&self, a: i32, b: i32) -> Obj00 {
        Obj00::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add01(&self, a: i32, b: i32) -> Obj01 {
        Obj01::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add02(&self, a: i32, b: i32) -> Obj02 {
        Obj02::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add03(&self, a: i32, b: i32) -> Obj03 {
        Obj03::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add04(&self, a: i32, b: i32) -> Obj04 {
        Obj04::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add05(&self, a: i32, b: i32) -> Obj05 {
        Obj05::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add06(&self, a: i32, b: i32) -> Obj06 {
        Obj06::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add07(&self, a: i32, b: i32) -> Obj07 {
        Obj07::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add08(&self, a: i32, b: i32) -> Obj08 {
        Obj08::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add09(&self, a: i32, b: i32) -> Obj09 {
        Obj09::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add10(&self, a: i32, b: i32) -> Obj10 {
        Obj10::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add11(&self, a: i32, b: i32) -> Obj11 {
        Obj11::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add12(&self, a: i32, b: i32) -> Obj12 {
        Obj12::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add13(&self, a: i32, b: i32) -> Obj13 {
        Obj13::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add14(&self, a: i32, b: i32) -> Obj14 {
        Obj14::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add15(&self, a: i32, b: i32) -> Obj15 {
        Obj15::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add16(&self, a: i32, b: i32) -> Obj16 {
        Obj16::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add17(&self, a: i32, b: i32) -> Obj17 {
        Obj17::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add18(&self, a: i32, b: i32) -> Obj18 {
        Obj18::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add19(&self, a: i32, b: i32) -> Obj19 {
        Obj19::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add20(&self, a: i32, b: i32) -> Obj20 {
        Obj20::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add21(&self, a: i32, b: i32) -> Obj21 {
        Obj21::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add22(&self, a: i32, b: i32) -> Obj22 {
        Obj22::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add23(&self, a: i32, b: i32) -> Obj23 {
        Obj23::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add24(&self, a: i32, b: i32) -> Obj24 {
        Obj24::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add25(&self, a: i32, b: i32) -> Obj25 {
        Obj25::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add26(&self, a: i32, b: i32) -> Obj26 {
        Obj26::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add27(&self, a: i32, b: i32) -> Obj27 {
        Obj27::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add28(&self, a: i32, b: i32) -> Obj28 {
        Obj28::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add29(&self, a: i32, b: i32) -> Obj29 {
        Obj29::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add30(&self, a: i32, b: i32) -> Obj30 {
        Obj30::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add31(&self, a: i32, b: i32) -> Obj31 {
        Obj31::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add32(&self, a: i32, b: i32) -> Obj32 {
        Obj32::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add33(&self, a: i32, b: i32) -> Obj33 {
        Obj33::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add34(&self, a: i32, b: i32) -> Obj34 {
        Obj34::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add35(&self, a: i32, b: i32) -> Obj35 {
        Obj35::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add36(&self, a: i32, b: i32) -> Obj36 {
        Obj36::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add37(&self, a: i32, b: i32) -> Obj37 {
        Obj37::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add38(&self, a: i32, b: i32) -> Obj38 {
        Obj38::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add39(&self, a: i32, b: i32) -> Obj39 {
        Obj39::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add40(&self, a: i32, b: i32) -> Obj40 {
        Obj40::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add41(&self, a: i32, b: i32) -> Obj41 {
        Obj41::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add42(&self, a: i32, b: i32) -> Obj42 {
        Obj42::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add43(&self, a: i32, b: i32) -> Obj43 {
        Obj43::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add44(&self, a: i32, b: i32) -> Obj44 {
        Obj44::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add45(&self, a: i32, b: i32) -> Obj45 {
        Obj45::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add46(&self, a: i32, b: i32) -> Obj46 {
        Obj46::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add47(&self, a: i32, b: i32) -> Obj47 {
        Obj47::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add48(&self, a: i32, b: i32) -> Obj48 {
        Obj48::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add49(&self, a: i32, b: i32) -> Obj49 {
        Obj49::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add50(&self, a: i32, b: i32) -> Obj50 {
        Obj50::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add51(&self, a: i32, b: i32) -> Obj51 {
        Obj51::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add52(&self, a: i32, b: i32) -> Obj52 {
        Obj52::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add53(&self, a: i32, b: i32) -> Obj53 {
        Obj53::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add54(&self, a: i32, b: i32) -> Obj54 {
        Obj54::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add55(&self, a: i32, b: i32) -> Obj55 {
        Obj55::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add56(&self, a: i32, b: i32) -> Obj56 {
        Obj56::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add57(&self, a: i32, b: i32) -> Obj57 {
        Obj57::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add58(&self, a: i32, b: i32) -> Obj58 {
        Obj58::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add59(&self, a: i32, b: i32) -> Obj59 {
        Obj59::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add60(&self, a: i32, b: i32) -> Obj60 {
        Obj60::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add61(&self, a: i32, b: i32) -> Obj61 {
        Obj61::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add62(&self, a: i32, b: i32) -> Obj62 {
        Obj62::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add63(&self, a: i32, b: i32) -> Obj63 {
        Obj63::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add64(&self, a: i32, b: i32) -> Obj64 {
        Obj64::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add65(&self, a: i32, b: i32) -> Obj65 {
        Obj65::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add66(&self, a: i32, b: i32) -> Obj66 {
        Obj66::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add67(&self, a: i32, b: i32) -> Obj67 {
        Obj67::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add68(&self, a: i32, b: i32) -> Obj68 {
        Obj68::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add69(&self, a: i32, b: i32) -> Obj69 {
        Obj69::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add70(&self, a: i32, b: i32) -> Obj70 {
        Obj70::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add71(&self, a: i32, b: i32) -> Obj71 {
        Obj71::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add72(&self, a: i32, b: i32) -> Obj72 {
        Obj72::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add73(&self, a: i32, b: i32) -> Obj73 {
        Obj73::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add74(&self, a: i32, b: i32) -> Obj74 {
        Obj74::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add75(&self, a: i32, b: i32) -> Obj75 {
        Obj75::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add76(&self, a: i32, b: i32) -> Obj76 {
        Obj76::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add77(&self, a: i32, b: i32) -> Obj77 {
        Obj77::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add78(&self, a: i32, b: i32) -> Obj78 {
        Obj78::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add79(&self, a: i32, b: i32) -> Obj79 {
        Obj79::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add80(&self, a: i32, b: i32) -> Obj80 {
        Obj80::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add81(&self, a: i32, b: i32) -> Obj81 {
        Obj81::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add82(&self, a: i32, b: i32) -> Obj82 {
        Obj82::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add83(&self, a: i32, b: i32) -> Obj83 {
        Obj83::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add84(&self, a: i32, b: i32) -> Obj84 {
        Obj84::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add85(&self, a: i32, b: i32) -> Obj85 {
        Obj85::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add86(&self, a: i32, b: i32) -> Obj86 {
        Obj86::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add87(&self, a: i32, b: i32) -> Obj87 {
        Obj87::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add88(&self, a: i32, b: i32) -> Obj88 {
        Obj88::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add89(&self, a: i32, b: i32) -> Obj89 {
        Obj89::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add90(&self, a: i32, b: i32) -> Obj90 {
        Obj90::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add91(&self, a: i32, b: i32) -> Obj91 {
        Obj91::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add92(&self, a: i32, b: i32) -> Obj92 {
        Obj92::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add93(&self, a: i32, b: i32) -> Obj93 {
        Obj93::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add94(&self, a: i32, b: i32) -> Obj94 {
        Obj94::new(a + b)
    }

    /// Returns the sum of a and b
    async fn add95(&self, a: i32, b: i32) -> Obj95 {
        Obj95::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add96(&self, a: i32, b: i32) -> Obj96 {
        Obj96::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add97(&self, a: i32, b: i32) -> Obj97 {
        Obj97::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add98(&self, a: i32, b: i32) -> Obj98 {
        Obj98::new(a + b)
    }
    /// Returns the sum of a and b
    async fn add99(&self, a: i32, b: i32) -> Obj99 {
        Obj99::new(a + b)
    }
}

pub fn create_schema() -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::new(Query, EmptyMutation, EmptySubscription)
}
