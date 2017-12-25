{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Control.Applicative ((<$>))
import Control.Monad (forM_)
import Data.Bits (xor, (.&.))
import Data.Char (ord)
import Data.Set as S (fromList, size, member, insert, empty)
import Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Maybe (isJust, fromJust)
import Data.Text as T (Text, strip, lines, words, unpack, splitOn)
import Data.Text.IO as T (readFile)
import Data.List (sortOn, group, foldl', elemIndex, iterate, delete, (\\))
import Debug.Trace (trace)
import Numeric (showHex)
import Text.Printf (printf)

(|>) :: a -> (a -> b) -> b
(|>) = flip ($)

readInt :: String -> Int
readInt = read

matches :: Int -> (Int, Int) -> Maybe (Int, Int)
matches pins (x, y)
    | pins == x || pins == y = Just (x, y)
    | otherwise = Nothing

parsePipe :: Text -> (Int, Int)
parsePipe s =
    let tokens = T.splitOn "/" s
        a = readInt (T.unpack (head tokens))
        b = readInt (T.unpack (tokens !! 1))
    in (min a b, max a b)

maxLength :: Int -> [(Int, Int)] -> Int -> Int -> (Int, Int)
maxLength lastPin ports currStrength currLength
    | null ports = (currLength, currStrength)
    | otherwise =
        let choices = [
                let outPin = if fst validPort == lastPin then snd validPort else fst validPort
                    remainingPorts = delete validPort ports
                    newLength = currLength + 1
                    newStrength = currStrength + fst validPort + snd validPort
                in maxLength outPin remainingPorts newStrength newLength
                | validPort <- map (matches lastPin) ports |> filter isJust |> map fromJust]
        in if null choices
        then (currLength, currStrength)
        else maximum choices

main :: IO ()
main = do
    input <- T.lines <$> T.readFile "24a.input"
    let pipes = map parsePipe input
    print $ snd $ maxLength 0 pipes 0 0

