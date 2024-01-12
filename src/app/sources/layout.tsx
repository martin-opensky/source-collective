'use client'

import { Button } from '@/components/catalyst/button'
import { Description, Field, Label } from '@/components/catalyst/fieldset'
import { Input } from '@/components/catalyst/input'
import {
  Listbox,
  ListboxLabel,
  ListboxOption,
} from '@/components/catalyst/listbox'
import { Textarea } from '@/components/catalyst/textarea'
import { ArticleIcon, AudioIcon, BookIcon, VideoIcon } from '@/utils/icons'
import { PlusIcon, MagnifyingGlassIcon } from '@heroicons/react/24/outline'

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex flex-col overflow-hidden">
      <Icons />
      <div className="hidden min-h-full flex-col border-r border-slate-100 lg:flex lg:w-72">
        <Form />
      </div>
      <div className="flex-1 overflow-hidden">{children}</div>
    </div>
  )
}

const Icons = () => {
  return (
    <div className="absolute m-1 flex gap-1">
      <Button outline className="rounded-xl bg-slate-800/10">
        <PlusIcon />
      </Button>
      <Button outline className="rounded-xl">
        <MagnifyingGlassIcon />
      </Button>
    </div>
  )
}

const Form = () => {
  return (
    <div className="m-2 mt-12 flex flex-col gap-3">
      <Field>
        <Label>Name</Label>
        {/* <Description>Example description here</Description> */}
        <Input name="name" />
      </Field>

      <Field>
        <Label>Description</Label>
        <Description>Optional: description for the source</Description>
        <Textarea name="description" />
      </Field>

      <Field>
        <Label>Source Type</Label>
        <Listbox name="source_type_id" placeholder="Select type&hellip;">
          <ListboxOption value="book">
            <BookIcon />
            <ListboxLabel>Book</ListboxLabel>
          </ListboxOption>
          <ListboxOption value="article">
            <ArticleIcon />
            <ListboxLabel>Article</ListboxLabel>
          </ListboxOption>
          <ListboxOption value="video">
            <VideoIcon />
            <ListboxLabel>Video</ListboxLabel>
          </ListboxOption>
          <ListboxOption value="audio">
            <AudioIcon />
            <ListboxLabel>Audio</ListboxLabel>
          </ListboxOption>
        </Listbox>
      </Field>

      <Field>
        <Label>URL</Label>
        <Description>Website for the source (if applicable)</Description>
        <Input name="url" type="url" />
      </Field>
    </div>
  )
}
