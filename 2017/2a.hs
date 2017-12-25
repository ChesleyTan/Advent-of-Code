{-# LANGUAGE OverloadedStrings #-}
import Data.Text as T (Text, strip, lines, words, unpack)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))

toInts :: Text -> Int
toInts = read . T.unpack

rowToInts :: [Text] -> [Int]
rowToInts = map toInts

main :: IO String
main = do
    input <- map (rowToInts . T.words) . T.lines . T.strip <$> T.readFile "2a.input"
    print $ sum $ map (\row -> maximum row - minimum row) input
    return ""

