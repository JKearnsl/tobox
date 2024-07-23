
import {Button} from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogTrigger,
} from "@/components/ui/alert-dialog"
import {Input} from "@/components/ui/input"
import {Label} from "@/components/ui/label"
import {useState} from "react";

function BasicLoginForm() {
  return (
      <>
        <div className="grid gap-2">
          <Label htmlFor="username">Username</Label>
          <Input
              id="username"
              type="username"
              placeholder="supauser"
              required
          />
        </div>
        <div className="grid gap-2">
          <div className="flex items-center">
            <Label htmlFor="password">Password</Label>
          </div>
          <Input id="password" type="password" required/>
        </div>
      </>
  )
}


function URILoginForm() {
  return (
      <>
        <div className="grid gap-2">
          <Label htmlFor="uri">URI</Label>
          <Input
              id="uri"
              type="text"
              placeholder="https://supauser:password@node.com"
                  required
              />
          </div>
      </>
  )
}


export function Auth() {
  const [isBasicForm, setIsBasicForm] = useState(true);

  const toggleForm = () => setIsBasicForm(!isBasicForm);

  return (
      <div className="flex flex-col justify-center items-center min-h-screen">
        <Card className="mx-auto max-w-sm">
          <CardHeader>
            <CardTitle className="text-2xl">Login</CardTitle>
            <CardDescription>
            {
                isBasicForm ?
                    "Enter your username below to login to your account" :
                    "Enter your URI below to login to your account"
              }
            </CardDescription>
          </CardHeader>

          <CardContent>
            <div className="grid gap-4">
              {isBasicForm ? <BasicLoginForm/> : <URILoginForm/>}

              <Button type="submit" className="w-full">
                Login
              </Button>

              <Button onClick={toggleForm} variant="outline" className="w-full">
                {isBasicForm ? "Login with URI" : "Login with username"}
              </Button>

            </div>
            <div className="mt-4 text-center text-sm">

                <AlertDialog>
                    <AlertDialogTrigger>How to get account</AlertDialogTrigger>
                    <AlertDialogContent>
                        <AlertDialogHeader>
                            <AlertDialogTitle>How to get account?</AlertDialogTitle>
                            <AlertDialogDescription>
                                Text stub
                            </AlertDialogDescription>
                        </AlertDialogHeader>
                        <AlertDialogFooter>
                            <AlertDialogAction>Ok</AlertDialogAction>
                        </AlertDialogFooter>
                    </AlertDialogContent>
                </AlertDialog>
            </div>
          </CardContent>
        </Card>
      </div>
  );
}
