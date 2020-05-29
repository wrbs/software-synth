# software-synth

First 2020 start of quarantine project - a software synthesizer and midi player in Rust.

Features:

- Unlimited polyphony
- Plays all midi files mostly correctly with mixed results for how good it sounds
- Some support for master volume and note velocity (although I haven't worked out what amplitude
  curve to use - it's currently linear)
- Some support for GM percussion (in that it knows that channel 10 is percussion and plays white
  noise instead of a note)

Doesn't feature:

- Timbral diversity - they're all basically some summed sin, square, triangle and sawtooth with a
  simple ADSR envelope and maybe some frequency modulation. There's been no real attempt to create
  instruments and map them to GM instruments at all (I think there's one brass sound that does
  that?)
- Configuration of "instruments" beyond editing the code and recompiling
- Diversity of percussion sounds - there's only one which is a short pulse of white noise
- Ability to play all midi files correctly - there's some bug to do with tempo that I never sorted
  out before I put the project on the back burner
- A mixer for each channel beyond whatever the midi file has put as the channel volume

I'm mainly releasing it as is without cleaning it up to put it out there: there's commented out
code, comments which no longer reflect whatever happens, and a general lack of comments,
explanation and documentation.

It's also pinned to use the master branch of `cpal` which may mean that it will break horribly the
next time someone looks at it. They've been working for months on an as of yet unreleased big
breaking change to their approach to concurrency that was much nicer to work with. When that gets
released I'll have to pin it to that.
