@ECHO OFF

FOR %%H IN (hash-read.txt nitrohash-read.txt CMS-read.txt NitroCMS-read.txt Cuckoo-read.txt NitroCuckoo-read.txt SpaceSaving-read.txt SpaceSaving-rap-read.txt NitroCompact-read.txt CMSNOMI-read.txt) do ECHO "" > %%H

FOR %%F IN ("c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago15.small.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago16Small.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago1610Mil.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago16.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\ny19A.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\ny19B.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\SJ14.small.txt") DO (
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type HASH --time-type READTIME >> hash-read.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type NitroHash --time-type READTIME >> nitrohash-read.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type CMS --time-type READTIME >> CMS-read.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type NitroCMS --time-type READTIME >> NitroCMS-read.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type Cuckoo --time-type READTIME >> Cuckoo-read.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type NitroCuckoo --time-type READTIME >> NitroCuckoo-read.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type SpaceSaving --time-type READTIME >> SpaceSaving-read.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type SpaceSaving --rap --time-type READTIME >> SpaceSaving-rap-read.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type NitroCuckoo --time-type READTIME --compact >> NitroCompact-read.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type CMS --time-type READTIME --avoid-mi >> CMSNOMI-read.txt
	)
)

