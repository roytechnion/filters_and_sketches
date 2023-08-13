@ECHO OFF

FOR %%H IN (NitroCompact.txt) do ECHO "" > %%H

FOR %%F IN ("c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago15.small.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago16Small.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago1610Mil.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago16.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\ny19A.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\ny19B.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\SJ14.small.txt") DO (
	FOR /L %%G IN (1,1,11) DO (
		cargo run --release -- --file-path %%F --ds-type NitroCuckoo --compare --compact >> NitroCompact.txt
	)
	FOR /L %%G IN (1,1,11) DO (
		cargo run --release -- --file-path %%F --ds-type NitroCuckoo --time_type READTIME --compact >> NitroCompact.txt
	)
	FOR /L %%G IN (1,1,11) DO (
		cargo run --release -- --file-path %%F --ds-type NitroCuckoo --time_type WRITETIME --compact >> NitroCompact.txt
	)
	FOR /L %%G IN (1,1,11) DO (
		cargo run --release -- --file-path %%F --ds-type NitroCuckoo --time_type RWTIME --compact >> NitroCompact.txt
	)
)

