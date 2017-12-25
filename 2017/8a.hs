{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Data.Set as S (fromList, size, member, insert, empty)
import qualified Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Text as T (Text, strip, lines, words, unpack)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))
import Data.List (sortOn, group)

parseLine [a, action, amt, _, b, operator, value] = (a, action, (read :: String -> Int) amt, b, operator, (read :: String -> Int) value)

adjustWithDefault k !newValue regs =
    let origValue = M.findWithDefault 0 k regs
    in M.insert k (origValue + newValue) regs

eval line regs = case line of
    (a, "inc", amt, b, "==", value) -> if M.findWithDefault 0 b regs == value then adjustWithDefault a amt regs else regs
    (a, "dec", amt, b, "==", value) -> if M.findWithDefault 0 b regs == value then adjustWithDefault a (-amt) regs else regs
    (a, "inc", amt, b, "!=", value) -> if M.findWithDefault 0 b regs /= value then adjustWithDefault a amt regs else regs
    (a, "dec", amt, b, "!=", value) -> if M.findWithDefault 0 b regs /= value then adjustWithDefault a (-amt) regs else regs
    (a, "inc", amt, b, ">", value)  -> if M.findWithDefault 0 b regs >  value then adjustWithDefault a amt regs else regs
    (a, "dec", amt, b, ">", value)  -> if M.findWithDefault 0 b regs >  value then adjustWithDefault a (-amt) regs else regs
    (a, "inc", amt, b, "<", value)  -> if M.findWithDefault 0 b regs <  value then adjustWithDefault a amt regs else regs
    (a, "dec", amt, b, "<", value)  -> if M.findWithDefault 0 b regs <  value then adjustWithDefault a (-amt) regs else regs
    (a, "inc", amt, b, ">=", value) -> if M.findWithDefault 0 b regs >= value then adjustWithDefault a amt regs else regs
    (a, "dec", amt, b, ">=", value) -> if M.findWithDefault 0 b regs >= value then adjustWithDefault a (-amt) regs else regs
    (a, "inc", amt, b, "<=", value) -> if M.findWithDefault 0 b regs <= value then adjustWithDefault a amt regs else regs
    (a, "dec", amt, b, "<=", value) -> if M.findWithDefault 0 b regs <= value then adjustWithDefault a (-amt) regs else regs
    _ -> error $ "Invalid line: " ++ show line

main :: IO String
main = do
    input <- (map (map T.unpack . T.words) . T.lines . T.strip) <$> T.readFile "8a.input"
    let parsedInput = map parseLine input
    -- print parsedInput
    print $ M.foldr max (minBound :: Int) $ foldr eval M.empty $ reverse parsedInput
    return ""
