{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Data.Set as S (fromList, size, member, insert, empty)
import Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Text as T (Text, strip, lines, words, unpack)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))
import Data.List (sortOn, group)

sumGroups "" _ totalScore _ totalGarbage = totalGarbage
sumGroups (x:xs) score totalScore inGarbage totalGarbage
    | not inGarbage && x == '{'     = sumGroups xs (score + 1) totalScore inGarbage totalGarbage
    | not inGarbage && x == '}'     = sumGroups xs (score - 1) (totalScore + score) inGarbage totalGarbage
    | not inGarbage && x == '<'     = sumGroups xs score totalScore True totalGarbage
    | inGarbage && x == '>'         = sumGroups xs score totalScore False totalGarbage
    | inGarbage && x == '!'         = sumGroups (tail xs) score totalScore inGarbage totalGarbage
    | inGarbage                     = sumGroups xs score totalScore inGarbage (totalGarbage + 1)
    | otherwise                     = sumGroups xs score totalScore inGarbage totalGarbage

main :: IO String
main = do
    input <- T.unpack . T.strip <$> T.readFile "9a.input"
    print $ sumGroups input 0 0 False 0
    return ""
