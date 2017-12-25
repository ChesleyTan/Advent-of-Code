{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Control.Monad (forM_)
import Data.Bits (xor)
import Data.Char (ord)
import Data.Set as S (fromList, size, member, insert, empty, union, difference, findMin)
import qualified Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Text as T (Text, strip, lines, words, unpack, pack, splitOn)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))
import Data.List (sortOn, group, foldl')
import Text.Printf (printf)
import Debug.Trace (trace)

(|>) = flip ($)
readInt = read :: String -> Int

parseLine line =
    let lineStr = T.unpack line
        depth = takeWhile (/= ':') lineStr |> readInt
        range = dropWhile (/= ' ') lineStr |> drop 1 |> readInt
    in (depth, range)

updateIdx idx newValue xs = (take idx xs) ++ [newValue] ++ (drop (idx + 1) xs)
bound lower upper x = max (min upper x) lower

pmod a b =
    let tmp = a `mod` b
    in if tmp < 0 then tmp + b else tmp

updateScanner idx layerMapping (scannerDir, scannerPos) =
    let range = M.findWithDefault (-1) idx layerMapping
        newDir = case (scannerDir, scannerPos) of
            (1, x) | x == range - 1 -> (-1)
            (-1, 0) -> 1
            (dir, _) -> dir
        newPos = bound 0 (range - 1) (scannerPos + newDir)
    in (newDir, newPos)

updateScanners :: [(Int, Int)] -> M.Map Int Int -> [(Int, Int)]
updateScanners scannerConfig layerMapping =
    map (\(idx, config) -> updateScanner idx layerMapping config) $ zip [0..] scannerConfig

--updateScannersPos layerMapping scannersDir scannersPos =
--    let range idx = M.findWithDefault (-1) idx layerMapping
--    in foldr (\(idx, pos) (dirs, positions) -> if (positions !! idx) == ((range idx) - 1) then (updateIdx idx (-1) dirs, updateIdx idx (pos) positions)) (scannersDir, scannersPos) $ zip [0..] scannersPos
--
--updateScannerPos layerMapping scannersDir (idx, pos)
--    | M.member idx layerMapping = (pos + 1) `mod` (M.findWithDefault 0 idx layerMapping)
--    | otherwise = -1

step :: M.Map Int Int -> Int -> (Int, [(Int, Int)], Int) -> (Int, [(Int, Int)], Int)
step _ _ (_, scannerConfig, _) | trace (show scannerConfig) False = undefined
step layerMapping deepestLayer (packetPos, scannerConfig, totalSeverity)
    | packetPos >= deepestLayer = (packetPos, scannerConfig, totalSeverity)
    | otherwise =
        let newScannerConfig = updateScanners scannerConfig layerMapping
            newSeverity =
                if scannerConfig !! (packetPos + 1) |> snd == 0
                    then totalSeverity + (packetPos + 1) * (M.findWithDefault 0 (packetPos + 1) layerMapping)
                    else totalSeverity
        in step layerMapping deepestLayer (packetPos + 1, newScannerConfig, newSeverity)

main :: IO String
main = do
    input <- map parseLine . T.lines . T.strip <$> T.readFile "13a.input"
    let deepestLayer = maximum $ map fst input
        layerMapping = M.fromList input
        initialConfig = [if M.member idx layerMapping then (-1, 0) else (0, -1) | idx <- [0..deepestLayer]]
    print $ step layerMapping deepestLayer (-1, initialConfig, 0)
    return ""
