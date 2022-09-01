module Main where

main::IO()
main = do
    putStrLn "Base of triangle"
    base <- readLn :: IO Double
    putStrLn "Height of triangle"
    height <- readLn :: IO Double
    putStrLn ("Area of triangle is" ++ show  (base*height/2))
