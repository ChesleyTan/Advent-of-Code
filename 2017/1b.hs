{-# LANGUAGE OverloadedStrings #-}
import Data.Char (digitToInt)
import Data.Text as T (Text, zip, strip, length, append, drop)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))

count :: Text -> Int
count s = foldr (\(x, y) acc -> if x == y then acc + digitToInt x else acc) 0
    $ T.zip s $ T.append (T.drop (T.length s `div` 2) s) s

main :: IO String
main = do
    input <- T.strip <$> T.readFile "1a.input"
    print $ count input
    return $ show $ count input

