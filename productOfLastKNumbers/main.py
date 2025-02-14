class ProductOfNumbers:

    def __init__(self):
        self.stream = []        

    def add(self, num: int) -> None:
        self.stream.append(num) 
        return

    def getProduct(self, k: int) -> int:
        product = 1 
        for i in range(0,k):
            product*=self.stream[-(i+1)]
        return product


  



