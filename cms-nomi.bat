@ECHO OFF

FOR %%H IN (CMSNOMI.txt) do ECHO "" > %%H

FOR %%F IN ("c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago15.small.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago16Small.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago1610Mil.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\Chicago16.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\ny19A.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\ny19B.txt" "c:\users\user\Dropbox (Technion Dropbox)\traces\dataCounters\SJ14.small.txt") DO (
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type CMS --compare --avoid-mi >> CMSNOMI.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type CMS --time-type READTIME --avoid-mi >> CMSNOMI.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type CMS --time-type WRITETIME --avoid-mi >> CMSNOMI.txt
	)
	FOR /L %%G IN (1,1,13) DO (
		cargo run --release -- --file-path %%F --ds-type CMS --time-type RWTIME --avoid-mi >> CMSNOMI.txt
	)
)

