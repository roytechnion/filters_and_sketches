@ECHO OFF

FOR %%H IN (hash-write.txt nitrohash-write.txt CMS-write.txt NitroCMS-write.txt Cuckoo-write.txt NitroCuckoo-write.txt SpaceSaving-write.txt SpaceSaving-rap-write.txt NitroCompact-write.txt CMSNOMI-write.txt) do ECHO "" > %%H

FOR %%F IN ("c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago15.small.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago16Small.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago1610Mil.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago16.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\ny19A.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\ny19B.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\SJ14.small.txt") DO (
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type HASH --time-type WRITETIME >> hash-write.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type NitroHash --time-type WRITETIME >> nitrohash-write.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type CMS --time-type WRITETIME >> CMS-write.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type NitroCMS --time-type WRITETIME >> NitroCMS-write.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type Cuckoo --time-type WRITETIME >> Cuckoo-write.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type NitroCuckoo --time-type WRITETIME >> NitroCuckoo-write.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type SpaceSaving --time-type WRITETIME >> SpaceSaving-write.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type SpaceSaving --rap --time-type WRITETIME >> SpaceSaving-rap-write.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type NitroCuckoo --time-type WRITETIME --compact >> NitroCompact-write.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type CMS --time-type WRITETIME --avoid-mi >> CMSNOMI-write.txt
	)
)

