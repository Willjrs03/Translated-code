import Foundation

//defining a function to convert the float into string, and ints
func roundAndConvert(number: Float) -> (string: String, float: Float, int: Int) {
    let rounded = round(number * 10) / 10
    return (String(rounded), rounded, Int(rounded))
}

//doing a loop to get input,convert the input into a float, and catch any errors that may come up
var x: Float = 0.0
while x == 0.0 {
    print("How many miles did they both travel for X: ")
    guard let xString = readLine() else { 
        print("Invalid input, please enter a number")
        continue
    }
    if let xValue = Float(xString) {
        x = xValue
    } else {
        print("Invalid input, please enter a number")
    }
}
//rauising to the second power
let x_2 = pow(x, 2)

//same here
var y: Float = 0.0
while y == 0.0 {
    print("How many miles did they both travel for Y: ")
    guard let yString = readLine() else { 
        print("Invalid input, please enter a number")
        continue
    }
    if let yValue = Float(yString) {
        y = yValue
    } else {
        print("Invalid input, please enter a number")
    }
}
let y_2 = pow(y, 2)

//adding
let combined = x_2 + y_2
//squaring
let squared = sqrt(combined)

//converting into target types
let (str, float, int) = roundAndConvert(number: squared)
print("String: \(str), Float: \(float), Int: \(int)")

//distance between both trains in the target types
let rnd_two = squared * 2
let (str_two, float_two, int_two) = roundAndConvert(number: rnd_two)
print("Distance between both trains is: ")
print("String: \(str_two), Float: \(float_two), Int: \(int_two)")
