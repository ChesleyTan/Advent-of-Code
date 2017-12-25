{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Control.Applicative ((<$>))
import Control.Monad (forM_)
import Data.Bits (xor, (.&.))
import Data.Char (ord)
import Data.Set as S (fromList, size, member, insert, empty)
import Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Text as T (Text, strip, lines, words, unpack, splitOn)
import Data.Text.IO as T (readFile)
import Data.List (sortOn, group, foldl', elemIndex, iterate)
import Debug.Trace (trace)
import Numeric (showHex)
import Text.Printf (printf)

(|>) :: a -> (a -> b) -> b
(|>) = flip ($)

primes :: [Int]
primes = [2, 3] ++ [x | x <- [4..], all (\d -> x `mod` d /= 0) $ takeWhile (\p -> p ^ 2 <= x) primes]

isPrime :: Int -> Bool
isPrime x = x `elem` takeWhile (<= x) primes

main :: IO ()
main = do
    let b = 109900
        c = 126900
        step = 17
    print $ length $ filter (not . isPrime) [b,b+step..c]


-- Annotated assembly:
--
--set b 99        #0 b = 99
--set c b         #1 c = b
--jnz a 2         #2 always happens if a = 1
--jnz 1 5         #3 jump to 8 (does not execute if a = 1)
--mul b 100       #4 b *= 100
--sub b -100000   #5 b += 100000
--set c b         #6 c = b
--sub c -17000    #7 c += 17000
--set f 1         #8 f = 1
--set d 2         #9 d = 2
--set e 2         #10 e = 2
--set g d         #11 g = d
--mul g e         #12 g *= e
--sub g b         #13 g -= b
--jnz g 2         #14
--set f 0         #15 f = 0 # occurs when b is factorized
--sub e -1        #16 e += 1
--set g e         #17 g = e
--sub g b         #18 g -= b
--jnz g -8        #19 jump to 11 (until e = b)
--sub d -1        # d += 1
--set g d         # g = d
--sub g b         # g -= b
--jnz g -13       # re-run 11-19 loop until d = b
--jnz f 2         #
--sub h -1        # h += 1
--set g b         # g = b
--sub g c         # g -= c
--jnz g 2         #
--jnz 1 3         #
--sub b -17       # b += 17
--jnz 1 -23       # jump to 8 (re-run program until b = c, with b += 17)
--
-- Decompiled:
-- a = 1
-- b = 99
-- c = b
-- d = 0
-- e = 0
-- f = 0
-- g = 0
-- h = 0
-- if (a != 0)
--      b *= 100
--      b += 100000
--      c = b
--      c += 17000
-- do-while (b != c)
--      f = 1
--      d = 2
--      do-while (d != b)
--          e = 2
--          do-while (e != b)
--              g = d * e - b
--              if (g == 0)
--                  f = 0
--              e += 1
--              g = e - b
--          d += 1
--          g = d - b
--
--      if (f == 0)
--          h += 1
--      g = b - c
--      if (g == 0)
--          return
--      b += 17
--
-- Decompiled optimized:
-- a = 1
-- b = 109900
-- c = 126900
-- d = 0
-- e = 0
-- f = 0
-- g = 0
-- h = 0
-- do-while (b != c)
--      f = 1
--      d = 2
--      do-while (d != b)
--          e = 2
--          do-while (e != b)
--              g = d * e - b
--              if (g == 0)
--                  f = 0
--              e += 1
--              g = e - b
--          d += 1
--          g = d - b
--
--      if (f == 0)
--          h += 1
--      b += 17
--
-- Logically simplified:
-- a = 1
-- b = 109900
-- c = 126900
-- d = 0
-- e = 0
-- f = 0
-- g = 0
-- h = 0
-- do-while (b != c)
--      f = 1 if b not prime else 0
--      b += 17
