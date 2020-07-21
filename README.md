# !Immense Work in Progress!
I am actively working on this, assume that anything and everything can and will change.




# Zkru

Zkru is a Zettelkasten tool. It takes a zettelkasten of notes (directory filled with .md files).

When run zkru runs it compiles all the md files into HTML, linking any note ID links that exist in files as well as applying markdown formatting/other custom formatting.

It then runs on a local webserver which allows the Zettelkasten to be viewed on HTML.





### FlashCard

The flashcard system is a separate compilation from the regular browser HTML. FlashCard goes through notes and from the key points in each one creates "flashcards" in order
to test retention of knowledge.

Testing can be started from either the top to test all knowledge, or from each individual note to test single note knowledge.
When test is clicked, you either choose or input a number and that is the number of flashcard tests you will have to complete (or the most amount available for any high #).

Each flashcard is fill in the blanks and will prompt if the answer is incorrect (maybe a hint option?).

Valid formatters/syntax:

# Specify id/other variables
The id declarations just needs to be something simple to set it at the top of each note (some sort of symbol combination doesn't really matter)
<id: d3ac6f05-8350-4895-b86c-c20aff39e5d0>
<title: Test Note>

# Specify tags
Since multiple tags are added I think that we should just use # to add tags.
#tag1 #tag2
#tag4 #tag3

# Add in links to other ZK notes
likely through  [[ID]], which in the compiled version would just be a hyperlink of that note's title.


# Answers / Key points in notes
Key points in notes should be denominated as such, either something like \aImportant part\a , what would happen in the compile stage
is that on each line start, we look for anything marked as such and compile it to *bold* in HTML, but to hide those as the answers during flashcard compilation while storing that as the answer for the flashcard fill in the blanks

"
- in Life the two most important things are \aLove\a and \aLegacy\a
"
would compile to

HTML:
- in Life the two most important things are **Love** and **Legacy**

FlashCard:
- in life the two most important things are <input>____</input> and <input>____</input>
<answers=love,legacy>

By adding in these key points to notes it makes it easier to sum each note up and to find important information when quickly skimming it. It is also benefitial as it will allow the flashcard system to test randomly to ensure that
the knowledge is not forgotton.
