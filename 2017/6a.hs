{-# LANGUAGE OverloadedStrings #-}
import Data.Set as S (fromList, size, member, insert, empty)
import Data.Map.Strict as M (fromList, size, findWithDefault, adjust)
import Data.Text as T (Text, strip, lines, words, unpack)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))
import Data.List (sort)

lmax [] currMax maxIdx idx = maxIdx
lmax (x:xs) currMax maxIdx idx = if x > currMax
    then lmax xs x idx (idx + 1)
    else lmax xs currMax maxIdx (idx + 1)

listUpdate xs idx newValue = (take idx xs) ++ [newValue] ++ (drop (idx + 1) xs)

ceilDiv a b = ceiling $ (fromIntegral a) / (fromIntegral b)

redistribute banks =
    let maxIdx = lmax banks (-1) 0 0
        blocks = banks !! maxIdx
        len = length banks
    in redistribute' (listUpdate banks maxIdx 0) blocks (ceilDiv blocks len) ((maxIdx + 1) `mod` len)

redistribute' banks total amt idx = if total <= 0
    then banks
    else redistribute' (listUpdate banks idx ((banks !! idx) + (min total amt))) (total - amt) amt ((idx + 1) `mod` (length banks))

run banks memo steps =
    if S.member banks memo
    then steps
    else run (redistribute banks) (S.insert banks memo) (steps + 1)

main :: IO String
main = do
    input <- map ((read :: String -> Int) . T.unpack) . T.words . T.strip <$> T.readFile "6a.input"
    print input
    print $ run input S.empty 0
    return ""
