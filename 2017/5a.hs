{-# LANGUAGE OverloadedStrings #-}
import Data.Set as S (fromList, size)
import Data.Map.Strict as M (fromList, size, findWithDefault, adjust)
import Data.Text as T (Text, strip, lines, words, unpack)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))
import Data.List (sort)

run pc instructions steps =
    if pc >= M.size instructions
        then steps
        else run (pc + M.findWithDefault 0 pc instructions) (M.adjust (+ 1) pc instructions) (steps + 1)

main :: IO String
main = do
    input <- map T.unpack . T.lines . T.strip <$> T.readFile "5a.input"
    print $ run 0 (M.fromList $ zip [0..] (map (read :: String -> Int) input)) 0
    return ""
