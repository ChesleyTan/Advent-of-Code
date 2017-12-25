{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Control.Monad (forM_)
import Data.Bits (xor)
import Data.Char (ord)
import Data.Set as S (fromList, size, member, insert, empty, union, difference, findMin)
import Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
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

--updateIdx idx newValue xs = (take idx xs) ++ [newValue] ++ (drop (idx + 1) xs)
--bound lower upper x = max (min upper x) lower
--
--pmod a b =
--    let tmp = a `mod` b
--    in if tmp < 0 then tmp + b else tmp
--
--updateScanner idx layerMapping (scannerDir, scannerPos) =
--    let range = M.findWithDefault (-1) idx layerMapping
--        newDir = case (scannerDir, scannerPos) of
--            (1, x) | x == range - 1 -> (-1)
--            (-1, 0) -> 1
--            (dir, _) -> dir
--        newPos = if scannerPos /= -1 then bound 0 (range - 1) (scannerPos + newDir) else -1
--    in (newDir, newPos)
--
--updateScanners :: [(Int, Int)] -> M.Map Int Int -> [(Int, Int)]
--updateScanners scannerConfig layerMapping =
--    map (\(idx, config) -> updateScanner idx layerMapping config) $ zip [0..] scannerConfig
--
--step :: M.Map Int Int -> Int -> Int -> (Int, [(Int, Int)], Int, Bool) -> (Int, [(Int, Int)], Int, Bool)
----step _ _ _ (packetPos, scannerConfig, _, caught) | trace (show (packetPos, scannerConfig, caught)) False = undefined
--step layerMapping deepestLayer !delay (!packetPos, !scannerConfig, !totalSeverity, !caught)
--    | caught = (packetPos, scannerConfig, totalSeverity, caught)
--    | packetPos >= deepestLayer = (packetPos, scannerConfig, totalSeverity, caught)
--    | otherwise =
--        let newScannerConfig = updateScanners scannerConfig layerMapping
--            newCaught = caught || (delay == 0 && scannerConfig !! (packetPos + 1) |> snd == 0)
--            newSeverity =
--                if (delay == 0) && scannerConfig !! (packetPos + 1) |> snd == 0
--                    then totalSeverity + (packetPos + 1) * (M.findWithDefault 0 (packetPos + 1) layerMapping)
--                    else totalSeverity
--        in step layerMapping deepestLayer (max 0 (delay - 1)) (if delay == 0 then packetPos + 1 else packetPos, newScannerConfig, newSeverity, newCaught)

futurePacketPositions delay deepestLayer = [delay..delay+deepestLayer]

futureScannerPos range timestamp =
    let next (position, direction)
            | (position + direction) >= range = (position - direction, -direction)
            | (position + direction) < 0 = (position - direction, -direction)
            | otherwise = (position + direction, direction)
        sim step (position, direction)
            | step == (timestamp `mod` (2 * range - 2)) = position
            | otherwise = sim (step + 1) (next (position, direction))
    in if range > 0 then sim 0 (0, 1) else (-1)

isCaught (range, timestamp) = futureScannerPos range timestamp == 0

main :: IO String
main = do
    input <- map parseLine . T.lines . T.strip <$> T.readFile "13a.input"
    let deepestLayer = maximum $ map fst input
        layerMapping = M.fromList input
    --  initialConfig = [if M.member idx layerMapping then (-1, 0) else (0, 1) | idx <- [0..deepestLayer]]
    --print $ step layerMapping 6 0 (-1, [(-1, 2), (1, 0), (0, -1), (0, -1), (-1, 2), (0, -1), (-1, 2)], 0, False)
    --print $ head $ dropWhile (\(_, (_, _, _, caught)) -> caught) [(delay, step layerMapping deepestLayer delay (-1, initialConfig, 0, False)) | delay <- [0..]]
        ranges = [M.findWithDefault (-1) idx layerMapping | idx <- [0..deepestLayer]]
        timestamps delay = futurePacketPositions delay deepestLayer
    print $ head $ dropWhile snd [(delay, any isCaught $ zip ranges (timestamps delay)) | delay <- [0..]]
    return ""
