use std::{ops::{Add, Neg,Mul,Sub,Div,Rem}, fmt::{Formatter,Debug}};

#[derive(Debug, Clone,Copy, PartialEq,Eq)]
pub enum Sign {
    Positive,
    Negative
}
impl Sign {
    pub fn flip(&mut self) {
        *self = match self {
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive
        };
    }
}

#[derive(Clone, PartialEq)]
pub struct NumericLiteral {
    sign:Sign,
    integer_part:usize,
    fractional_part:Option<(usize,usize)>,
}

impl NumericLiteral {
    pub fn negate(&mut self){
        self.sign.flip()
    }
    pub fn new(sign:Sign, mut integer_part:usize, mut fractional_part:Option<(usize,usize)>)->Self{
        if let Some((n,d)) = fractional_part {
            let n_rem = n%d;
            let n_whole = n/d;
            fractional_part = Some((n_rem,d));
            integer_part+=n_whole;
        }
        Self {
            sign,
            integer_part,
            fractional_part,
        }
    }
}

impl From<NumericLiteral> for f64 {
    fn from(NumericLiteral{sign,integer_part,fractional_part}: NumericLiteral) -> Self {
        let mut f = (integer_part as Self) + fractional_part.map(|(n,d)|(n as Self)/(d as Self)).unwrap_or_default();
        if sign == Sign::Negative {
            f=f.neg()
        };
        f
    }
}
impl From<isize> for NumericLiteral {
    fn from(integer: isize) -> Self {
        if integer < 0 {
            Self {
                sign: Sign::Negative,
                integer_part: -integer as _,
                fractional_part: None
            }
        }else {
            Self {
                sign: Sign::Positive,
                integer_part: integer as _,
                fractional_part: None
            }
        }
    }
}


impl Add for NumericLiteral {
    type Output = NumericLiteral;

    fn add(self, rhs: Self) -> Self::Output {
        if self.sign==rhs.sign {
            // todo!();
            Self::new(self.sign, self.integer_part + rhs.integer_part, self.fractional_part.zip(rhs.fractional_part).map(|((l0,l1),(r0,r1))|{
                (l0*r1+l1*r0 ,l1*r1)
            }))
        } else {
            let (mut sign,diff) = if rhs.integer_part < self.integer_part {
                (Sign::Positive,self.integer_part - rhs.integer_part)
            } else {
                (Sign::Negative, rhs.integer_part-self.integer_part)
            };
            let frac_parts = match (self.fractional_part,rhs.fractional_part) {
                (Some((a,b)),None) => Some(((a,0),b)),
                (None,Some((a,b))) => Some(((0,a),b)),
                (Some((a,b)),Some((c,d))) if b == d => Some(((a,c),d)),
                (Some((a,b)),Some((c,d))) => Some(((a*d,c*b),b*d)),
                _ => None
            }.map(|((n_self,n_rhs),d)|{
                let (n_sign,n)=if n_rhs< n_self {
                    (Sign::Positive,n_self- n_rhs)
                } else {
                    (Sign::Negative, n_rhs-n_self)
                };
                let n_rem = n%d;
                let n_whole = n/d;
                // dbg!(n_sign,n_whole,(n_rem,d),n);
                (n_sign,n_whole,(n_rem,d))
            });

            if let Some((f_sign, f_whole, frac)) = frac_parts {
                if f_sign == sign {
                    Self {
                        sign,
                        integer_part:diff+f_whole,
                        fractional_part:Some(frac)
                    }
                }else if f_whole < diff {
                    Self {
                        sign,
                        integer_part: diff-f_whole,
                        fractional_part:Some(frac)
                    }
                }else {
                    sign.flip();
                    Self {
                        sign,
                        integer_part: f_whole-diff,
                        fractional_part:Some(frac)
                    }
                }
            }else {
                Self{
                    sign,
                    integer_part:diff,
                    fractional_part:self.fractional_part
                }
            }

            // todo!();
            // let a  =match 0.cmp(int_diff) {
            //     Ordering::Less => {}
            //     Ordering::Equal => {}
            //     Ordering::Greater => {}
            // };

            // let self_frac = (self.fractional_part.0*rhs.fractional_part.1,self.fractional_part.1*rhs.fractional_part.1);
            // let frac_diff = self.fractional_part.0 - rhs.fractional_part.0;
            // let (sign) = if int_diff > 0 {
            //     self.sign
            // } else {
            //     rhs.sign
            // };
            // Self {
            //     sign: self.sign,
            //     integer_part: self.integer_part + rhs.integer_part,
            //     fractional_part: self.fractional_part.zip(rhs.fractional_part).map(|(l0,l1),(r0,r1)|{
            //         (l0*r1+l1*r0 ,l1*r1)
            //     })
            // }
        }
    }
}
impl Mul for NumericLiteral {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output{todo!()}
}
impl Div for NumericLiteral {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output{todo!()}
}
impl Sub for NumericLiteral {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output{todo!()}
}
impl Rem for NumericLiteral {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output{todo!()}
}

impl Debug for NumericLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.sign == Sign::Negative {
            write!(f, "-")?;
        }
        write!(f,"{}",self.integer_part)?;
        if let Some((n,d)) = self.fractional_part {
            write!(f," {}/{}",n,d)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use super::NumericLiteral;
 




    #[test]
    fn add_same_sign(){
        let a:NumericLiteral = 9.into();
        let b:NumericLiteral = 14.into();
        assert_eq!(a+b, NumericLiteral{
            sign:super::Sign::Positive,
            integer_part: 23,
            fractional_part:None
        })
    }

    #[test]
    fn add_diff_sign(){
        let a:NumericLiteral = 9.into();
        let b:NumericLiteral = (-14).into();
        assert_eq!(a+b, NumericLiteral{
            sign:super::Sign::Negative,
            integer_part: 5,
            fractional_part:None
        })
    }


    #[test]
    fn add_diff_sign_frac(){
        let a = NumericLiteral{
            sign:super::Sign::Positive,
            integer_part: 1,
            fractional_part:Some((2,3))
        };
        let b = NumericLiteral{
            sign:super::Sign::Negative,
            integer_part: 3,
            fractional_part:Some((9,8))
        };
        assert_eq!(a+b, NumericLiteral{
            sign:super::Sign::Negative,
            integer_part: 2,
            fractional_part:Some((11,24))
        })
    }

    #[test]
    fn add_same_sign_frac(){
        let a = NumericLiteral{
            sign:super::Sign::Positive,
            integer_part: 1,
            fractional_part:Some((2,3))
        };
        let b = NumericLiteral{
            sign:super::Sign::Positive,
            integer_part: 3,
            fractional_part:Some((9,8))
        };
        assert_eq!(a+b, NumericLiteral{
            sign:super::Sign::Positive,
            integer_part: 5,
            fractional_part:Some((19,24))
        })
    }
}