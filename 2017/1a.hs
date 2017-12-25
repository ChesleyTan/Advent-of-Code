{-# LANGUAGE OverloadedStrings #-}
import Data.Char (digitToInt)
import Data.Text as T (Text, zip, tail, strip, length, index, snoc)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))

count :: Text -> Int
count s = foldr (\(x, y) acc -> if x == y then acc + digitToInt x else acc) 0 $ T.zip s $ T.tail s

main :: IO String
main = do
    input <- T.strip <$> T.readFile "1a.input"
    print input
    return $ show $ count $ T.snoc input $ input `T.index` (T.length input - 1)

