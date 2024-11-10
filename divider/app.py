import divider
from divider.types import Err

class Divide(divider.Divider):
    def divide(self, a: float, b: float) -> float:
        if b == 0.0:
            raise Err("Denominator is equal to zero!")
        return a / b
