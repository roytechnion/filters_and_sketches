@ECHO OFF

FOR %%H IN (hash-compare.txt nitrohash-compare.txt CMS-compare.txt NitroCMS-compare.txt Cuckoo-compare.txt NitroCuckoo-compare.txt SpaceSaving-compare.txt SpaceSaving-rap-compare.txt NitroCompact-compare.txt CMSNOMI-compare.txt) do ECHO "" > %%H

FOR %%F IN ("c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago15.small.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago16Small.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago1610Mil.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago16.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\ny19A.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\ny19B.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\SJ14.small.txt") DO (
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type HASH --compare >> hash-compare.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type NitroHash --compare >> nitrohash-compare.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type CMS --compare >> CMS-compare.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type NitroCMS --compare >> NitroCMS-compare.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type Cuckoo --compare >> Cuckoo-compare.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type NitroCuckoo --compare >> NitroCuckoo-compare.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type SpaceSaving --compare >> SpaceSaving-compare.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type SpaceSaving --rap --compare >> SpaceSaving-rap-compare.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type NitroCuckoo --compare --compact >> NitroCompact-compare.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type CMS --compare --avoid-mi >> CMSNOMI-compare.txt
	)
)

