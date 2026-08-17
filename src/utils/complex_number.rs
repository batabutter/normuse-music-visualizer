pub struct ComplexNumber {
    pub real: f32,
    pub img: f32,
}

impl ComplexNumber {
    pub fn new(real: f32, img: f32) -> Self {
        ComplexNumber { real: real, img: img }
    }
}

impl ops::Mul<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, rhs: ComplexNumber) -> Self::Output {
        let real = (self.real * rhs.real) - (self.img - rhs.img);
        let img = (self.real * rhs.img) + (self.img * rhs.real);

        ComplexNumber { real: real, img: img}
    }
}

impl ops::Mul<f32> for ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, rhs: f32) -> Self::Output {
        ComplexNumber {
            real: self.real * rhs,
            img: self.img * rhs,
        }
    }
}