@ECHO OFF

FOR %%H IN (hash-readwrite.txt nitrohash-readwrite.txt CMS-readwrite.txt NitroCMS-readwrite.txt Cuckoo-readwrite.txt NitroCuckoo-readwrite.txt SpaceSaving-readwrite.txt SpaceSaving-rap-readwrite.txt) do ECHO "" > %%H

FOR %%F IN ("c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago15.small.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago16Small.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago1610Mil.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago16.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\ny19A.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\ny19B.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\SJ14.small.txt") DO (
	FOR /L %%G IN (1,1,11) DO (
		cargo run --release -- --file-path %%F --ds-type HASH --time-type RWTIME >> hash-readwrite.txt
	)
	FOR /L %%G IN (1,1,11) DO (
		cargo run --release -- --file-path %%F --ds-type NitroHash --time-type RWTIME >> nitrohash-readwrite.txt
	)
	FOR /L %%G IN (1,1,11) DO (
		cargo run --release -- --file-path %%F --ds-type CMS --time-type RWTIME >> CMS-readwrite.txt
	)
	FOR /L %%G IN (1,1,11) DO (
		cargo run --release -- --file-path %%F --ds-type NitroCMS --time-type RWTIME >> NitroCMS-readwrite.txt
	)
	FOR /L %%G IN (1,1,11) DO (
		cargo run --release -- --file-path %%F --ds-type Cuckoo --time-type RWTIME >> Cuckoo-readwrite.txt
	)
	FOR /L %%G IN (1,1,11) DO (
		cargo run --release -- --file-path %%F --ds-type NitroCuckoo --time-type RWTIME >> NitroCuckoo-readwrite.txt
	)
	FOR /L %%G IN (1,1,11) DO (
		cargo run --release -- --file-path %%F --ds-type SpaceSaving --time-type RWTIME >> SpaceSaving-readwrite.txt
	)
	FOR /L %%G IN (1,1,11) DO (
		cargo run --release -- --file-path %%F --ds-type SpaceSaving --rap --time-type RWTIME >> SpaceSaving-rap-readwrite.txt
	)
)

