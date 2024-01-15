#!/bin/sh
/usr/local/bin/iboodrss -c nl -l nl > /web/nl-nl.xml &
/usr/local/bin/iboodrss -c be -l nl > /web/be-nl.xml &
/usr/local/bin/iboodrss -c be -l fr > /web/be-fr.xml &
/usr/local/bin/iboodrss -c de -l de > /web/de-de.xml &
/usr/local/bin/iboodrss -c at -l de > /web/at-de.xml &
/usr/local/bin/iboodrss -c pl -l pl > /web/pl-pl.xml &
/usr/local/bin/iboodrss -c fr -l fr > /web/fr-fr.xml &