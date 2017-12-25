{-# LANGUAGE OverloadedStrings #-}
import Data.Set as S (fromList, size)
import Data.Text as T (Text, strip, lines, words, unpack)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))
import Data.List (sort)

main :: IO String
main = do
    input <- map (map (sort . T.unpack) . T.words) . T.lines . T.strip <$> T.readFile "4a.input"
    print $ length $ filter (\x -> S.size (S.fromList x) == length x) input
    return ""
