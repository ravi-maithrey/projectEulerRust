module Main where

main::IO()
main = do
    putStrLn "Base of triangle"
    base <- read getLine
    putStrLn "Height of triangle"
    height <- read getLine
    putStrLn ("Area of triangle is" ++ show $ 0.5*base*height)
