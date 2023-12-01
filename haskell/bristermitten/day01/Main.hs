

module Main where
import qualified Data.Char as Char
import Data.Maybe (maybeToList)
import Data.List (tails, find, isPrefixOf)

getInput :: IO String
getInput = readFile "input.txt"

firstAndLast :: [b] -> (b, b)
firstAndLast [] = error "No digits found"
firstAndLast [x] = (x, x)
firstAndLast xs = (head xs, last xs)

process :: String -> Int
process = uncurry ((+) . (10 *)) . firstAndLast . map (read . pure) . filter Char.isDigit


process2 :: String -> Int
process2 = uncurry ((+) . (10 *)) . firstAndLast . concatMap runParseParts . tails
    where
        runParseParts :: String -> [Int]
        runParseParts [] = []
        runParseParts xs = let (n, rest) = parseParts xs in maybeToList n ++ runParseParts rest

        prefixes = map (\s -> (show @Int s, s)) [0..9] ++
            [
            ("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)]

        parseParts :: String -> (Maybe Int, String)
        parseParts [] = (Nothing, [])
        parseParts str = case find ((`isPrefixOf` str) . fst) prefixes of
                    Just (p, n) -> (Just n, drop (length p) str)
                    Nothing -> (Nothing, tail str)

main :: IO ()
main = do
    input <- lines <$> getInput
    print (sum (map process input))
    print (sum (map process2 input))